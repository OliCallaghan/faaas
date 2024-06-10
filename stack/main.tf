terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.53"
    }
  }
  required_version = ">= 1.2.0"
}

provider "aws" {
  region = "eu-west-2"
}

resource "aws_elasticache_serverless_cache" "redis" {
  engine = "redis"
  name   = "redis"

  cache_usage_limits {
    data_storage {
      maximum = 5
      unit    = "GB"
    }
    ecpu_per_second {
      maximum = 5000
    }
  }

  daily_snapshot_time      = "09:00"
  description              = "Redis Server"
  major_engine_version     = "7"
  snapshot_retention_limit = 1
  security_group_ids       = [aws_security_group.redis.id]
  subnet_ids = [
    aws_subnet.private_subnet_1.id,
    aws_subnet.private_subnet_2.id,
  ]
}

resource "aws_security_group" "redis" {
  name   = "redis_sg"
  vpc_id = aws_vpc.main.id

  ingress {
    from_port   = 6379
    to_port     = 6379
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 6380
    to_port     = 6380
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 6379
    to_port     = 6379
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 6380
    to_port     = 6380
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "redis"
  }
}

resource "aws_security_group" "lambda_sg" {
  name   = "lambda_sg"
  vpc_id = aws_vpc.main.id

  ingress {
    from_port       = 0
    to_port         = 0
    protocol        = "-1"
    security_groups = [aws_security_group.redis.id]
    cidr_blocks     = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "lambda_sg"
  }
}



resource "aws_lambda_function" "handler" {
  filename = "${local.build_path}/handler.zip"
  handler  = "handler.entrypoint"
  runtime  = "nodejs20.x"
  role     = aws_iam_role.iam_for_lambda.arn

  memory_size = 256

  source_code_hash = filebase64sha256("${local.build_path}/handler.zip")

  function_name = "handler"
  timeout       = 30

  environment {
    variables = {
      MQ_HOST   = "b-5a738d61-9443-4566-84e9-2c576774f230.mq.eu-west-2.amazonaws.com"
      MQ_PORT   = "5671"
      MQ_USER   = "admin"
      MQ_PASS   = "ishouldmakethissecure"
      REDIS_URL = "rediss://redis-av1ecw.serverless.euw2.cache.amazonaws.com:6379"
    }
  }

  vpc_config {
    security_group_ids = [aws_security_group.lambda_sg.id]
    subnet_ids = [
      aws_subnet.private_subnet_1.id,
      aws_subnet.private_subnet_2.id,
    ]
  }
}

resource "aws_lambda_event_source_mapping" "handler_rabbit_mq_event_source" {
  batch_size       = 1
  event_source_arn = aws_mq_broker.rabbit_mq.arn
  enabled          = true
  function_name    = aws_lambda_function.handler.arn
  queues           = ["task_invocations"]

  source_access_configuration {
    type = "VIRTUAL_HOST"
    uri  = "/"
  }

  source_access_configuration {
    type = "BASIC_AUTH"
    uri  = aws_secretsmanager_secret_version.rabbit_mq_auth.arn
  }
}

resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda"

  assume_role_policy = <<EOF
    {
    "Version": "2012-10-17",
    "Statement": [
        {
        "Action": "sts:AssumeRole",
        "Principal": {
            "Service": "lambda.amazonaws.com"
        },
        "Effect": "Allow",
        "Sid": ""
        }
    ]
    }
    EOF

  inline_policy {
    name = "rabbitmq_access"

    policy = jsonencode({
      "Version" : "2012-10-17",
      "Statement" : [
        {
          "Action" : [
            "mq:DescribeBroker",
            "secretsmanager:GetSecretValue",
            "ec2:CreateNetworkInterface",
            "ec2:DeleteNetworkInterface",
            "ec2:DescribeNetworkInterfaces",
            "ec2:DescribeSecurityGroups",
            "ec2:DescribeSubnets",
            "ec2:DescribeVpcs",
            "elasticache:Connect",
            "logs:CreateLogGroup",
            "logs:CreateLogStream",
            "logs:PutLogEvents"
          ],
          "Resource" : "*",
          "Effect" : "Allow"
        }
      ]
    })
  }
}

resource "aws_iam_role" "iam_for_ecs" {
  name = "iam_for_ecs"

  assume_role_policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : "sts:AssumeRole",
        "Principal" : {
          "Service" : "ec2.amazonaws.com"
        },
        "Effect" : "Allow",
        "Sid" : ""
      }
    ]
  })
}

resource "aws_mq_broker" "rabbit_mq" {
  broker_name = "rabbitmq"

  engine_type        = "RabbitMQ"
  engine_version     = "3.12.13"
  host_instance_type = "mq.t3.micro"
  # security_groups     = [aws_security_group.rabbit_mq.id]
  publicly_accessible = true

  # subnet_ids = [
  #   aws_subnet.public_subnet_1.id,
  #   aws_subnet.public_subnet_2.id,
  # ]

  user {
    username = "admin"
    password = "ishouldmakethissecure"
  }
}

resource "aws_default_vpc" "default" {
  tags = {
    Name = "Default VPC"
  }
}

resource "aws_security_group" "rabbit_mq" {
  vpc_id = aws_vpc.main.id
}

resource "aws_secretsmanager_secret" "rabbit_mq_auth" {
  name = "rabbit-mq-auth"
}

resource "aws_secretsmanager_secret_version" "rabbit_mq_auth" {
  secret_id = aws_secretsmanager_secret.rabbit_mq_auth.id
  secret_string = jsonencode({
    username = "admin",
    password = "ishouldmakethissecure"
  })
}

import {
  to = aws_ecr_repository.faaas_gateway
  id = "faaas_gateway"
}

import {
  to = aws_ecr_repository.postgres_faaas_engine
  id = "postgres_faaas_engine"
}

resource "aws_ecr_repository" "faaas_gateway" {
  name                 = "faaas_gateway"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}

resource "aws_ecr_repository" "postgres_faaas_engine" {
  name                 = "postgres_faaas_engine"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}

###########################

# Create VPC
resource "aws_vpc" "main" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_support   = true
  enable_dns_hostnames = true

  tags = {
    Name = "main-vpc"
  }
}

# Create Internet Gateway
resource "aws_internet_gateway" "gw" {
  vpc_id = aws_vpc.main.id

  tags = {
    Name = "main-gw"
  }
}

# Create Public Subnet 1
resource "aws_subnet" "public_subnet_1" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "10.0.1.0/24"
  availability_zone       = "eu-west-2a"
  map_public_ip_on_launch = true

  tags = {
    Name = "public-subnet-1"
  }
}

# Create Public Subnet 2
resource "aws_subnet" "public_subnet_2" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "10.0.2.0/24"
  availability_zone       = "eu-west-2b"
  map_public_ip_on_launch = true

  tags = {
    Name = "public-subnet-2"
  }
}

# Create Private Subnet 1
resource "aws_subnet" "private_subnet_1" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = "10.0.3.0/24"
  availability_zone = "eu-west-2a"

  tags = {
    Name = "private-subnet-1"
  }
}

# Create Private Subnet 2
resource "aws_subnet" "private_subnet_2" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = "10.0.4.0/24"
  availability_zone = "eu-west-2b"

  tags = {
    Name = "private-subnet-2"
  }
}

# Create NAT Gateway in Public Subnet 1
resource "aws_eip" "nat" {
  vpc = true
}

resource "aws_nat_gateway" "nat_gw" {
  allocation_id = aws_eip.nat.id
  subnet_id     = aws_subnet.public_subnet_1.id

  tags = {
    Name = "main-nat-gw"
  }

  depends_on = [aws_internet_gateway.gw]
}

# Create Route Table for Public Subnets
resource "aws_route_table" "public" {
  vpc_id = aws_vpc.main.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.gw.id
  }

  tags = {
    Name = "public-rt"
  }
}

# Associate Route Table with Public Subnets
resource "aws_route_table_association" "public-rt-association-1" {
  subnet_id      = aws_subnet.public_subnet_1.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public-rt-association-2" {
  subnet_id      = aws_subnet.public_subnet_2.id
  route_table_id = aws_route_table.public.id
}

# Create Route Table for Private Subnets
resource "aws_route_table" "private" {
  vpc_id = aws_vpc.main.id

  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.nat_gw.id
  }

  tags = {
    Name = "private-rt"
  }
}

# Associate Route Table with Private Subnets
resource "aws_route_table_association" "private-rt-association-1" {
  subnet_id      = aws_subnet.private_subnet_1.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private-rt-association-2" {
  subnet_id      = aws_subnet.private_subnet_2.id
  route_table_id = aws_route_table.private.id
}

# Create Security Group
resource "aws_security_group" "ecs_sg" {
  vpc_id = aws_vpc.main.id

  description = "Allow traffic to ECS tasks"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "ecs-sg"
  }
}

# Create IAM role for ECS task execution
resource "aws_iam_role" "ecs_task_execution_role" {
  name = "ecs_task_execution_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow",
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      }
    ]
  })

  tags = {
    Name = "ecs_task_execution_role"
  }
}

# Attach the AmazonECSTaskExecutionRolePolicy policy to the IAM role
resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# Define ECS Cluster
resource "aws_ecs_cluster" "main" {
  name = "main-cluster"
}

resource "aws_cloudwatch_log_group" "faaas_gateway" {
  name = local.gateway_log_group
}

resource "aws_cloudwatch_log_group" "faaas_proxy_sql_pg" {
  name = local.proxy_sql_pg_log_group
}

# Define ECS Task Definition
resource "aws_ecs_task_definition" "app" {
  family                   = "app-task"
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  container_definitions = jsonencode([{
    name  = "app"
    image = "${aws_ecr_repository.faaas_gateway.repository_url}:latest"
    portMappings = [{
      containerPort = 80
      hostPort      = 80
    }]
    environment = [
      {
        name  = "MQ_HOST"
        value = "b-5a738d61-9443-4566-84e9-2c576774f230.mq.eu-west-2.amazonaws.com"
      },
      {
        name  = "MQ_PORT"
        value = "5671"
      },
      {
        name  = "MQ_USER"
        value = "admin"
      },
      {
        name  = "MQ_PASS"
        value = "ishouldmakethissecure"
      }
    ]
    logConfiguration = {
      logDriver = "awslogs"
      options = {
        "awslogs-create-group" : "true"
        "awslogs-group" : local.gateway_log_group
        "awslogs-region" : "eu-west-2"
        "awslogs-stream-prefix" : "ecs"
      }
    }
  }])

  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "ARM64"
  }

  tags = {
    Name = "app-task"
  }
}

# Define ECS Task Definition
resource "aws_ecs_task_definition" "proxy_sql_pg" {
  family                   = "proxy-sql-pg"
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  container_definitions = jsonencode([{
    name  = "proxy-sql-pg"
    image = "${aws_ecr_repository.postgres_faaas_engine.repository_url}:latest"
    environment = [
      {
        name  = "MQ_HOST"
        value = "b-5a738d61-9443-4566-84e9-2c576774f230.mq.eu-west-2.amazonaws.com"
      },
      {
        name  = "MQ_PORT"
        value = "5671"
      },
      {
        name  = "MQ_USER"
        value = "admin"
      },
      {
        name  = "MQ_PASS"
        value = "ishouldmakethissecure"
      },
      {
        name  = "PG_HOST"
        value = "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com"
      },
      {
        name  = "PG_PORT"
        value = "5432"
      },
      {
        name  = "PG_USER"
        value = "faaasuser"
      },
      {
        name  = "PG_PASS"
        value = "securepassword"
      },
      {
        name  = "PG_DB"
        value = "postgres"
      }
    ]
    logConfiguration = {
      logDriver = "awslogs"
      options = {
        "awslogs-create-group" : "true"
        "awslogs-group" : local.proxy_sql_pg_log_group
        "awslogs-region" : "eu-west-2"
        "awslogs-stream-prefix" : "ecs"
      }
    }
  }])

  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "ARM64"
  }

  tags = {
    Name = "proxy-sql-pg-task"
  }
}

# Create Application Load Balancer
resource "aws_lb" "app" {
  name               = "app-lb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.ecs_sg.id]
  subnets = [
    aws_subnet.public_subnet_1.id,
    aws_subnet.public_subnet_2.id
  ]

  tags = {
    Name = "app-lb"
  }
}

# Create target group
resource "aws_lb_target_group" "app" {
  name        = "app-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = aws_vpc.main.id
  target_type = "ip"

  health_check {
    path                = "/health-check"
    port                = "80"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 2
    unhealthy_threshold = 2
  }

  tags = {
    Name = "app-tg"
  }
}

# Create listener
resource "aws_lb_listener" "app" {
  load_balancer_arn = aws_lb.app.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app.arn
  }
}

# Create ECS Service
resource "aws_ecs_service" "app" {
  name                 = "app-service"
  cluster              = aws_ecs_cluster.main.id
  task_definition      = aws_ecs_task_definition.app.arn
  desired_count        = 1
  launch_type          = "FARGATE"
  force_new_deployment = true

  network_configuration {
    subnets = [
      aws_subnet.private_subnet_1.id,
      aws_subnet.private_subnet_2.id
    ]
    security_groups  = [aws_security_group.ecs_sg.id]
    assign_public_ip = false
  }
  load_balancer {
    target_group_arn = aws_lb_target_group.app.arn
    container_name   = "app"
    container_port   = 80
  }

  depends_on = [aws_lb_listener.app]

  tags = {
    Name = "app-service"
  }
}

resource "aws_ecs_service" "proxy_sql_pg" {
  name                 = "proxy-sql-pg-service"
  cluster              = aws_ecs_cluster.main.id
  task_definition      = aws_ecs_task_definition.proxy_sql_pg.arn
  desired_count        = 1
  launch_type          = "FARGATE"
  force_new_deployment = true

  network_configuration {
    subnets = [
      aws_subnet.private_subnet_1.id,
      aws_subnet.private_subnet_2.id
    ]
    security_groups  = [aws_security_group.ecs_sg.id]
    assign_public_ip = false
  }

  tags = {
    Name = "proxy-sql-pg-service"
  }
}

variable "aws_region" {
  default = "eu-west-2"
}

resource "aws_db_subnet_group" "postgres" {
  name = "postgres"
  subnet_ids = [
    aws_subnet.public_subnet_1.id,
    aws_subnet.public_subnet_2.id,
  ]

  tags = {
    Name = "postgres-db"
  }
}

resource "aws_security_group" "postgres" {
  name   = "postgres_sg"
  vpc_id = aws_vpc.main.id

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "postgres-db"
  }
}

resource "aws_db_instance" "postgres" {
  identifier             = "postgres-db"
  allocated_storage      = 20
  engine                 = "postgres"
  engine_version         = "16.3"
  instance_class         = "db.t3.micro"
  username               = "faaasuser"
  password               = "securepassword"
  parameter_group_name   = "default.postgres16"
  publicly_accessible    = true
  db_subnet_group_name   = aws_db_subnet_group.postgres.name
  vpc_security_group_ids = [aws_security_group.postgres.id]
  skip_final_snapshot    = true

  tags = {
    Name = "postgres-db"
  }
}
