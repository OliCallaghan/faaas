resource "aws_ecs_service" "pg_proxy" {
  name                 = "pg-proxy-${var.db_name}"
  cluster              = var.cluster_id
  task_definition      = aws_ecs_task_definition.pg_proxy.arn
  desired_count        = 1
  launch_type          = "FARGATE"
  force_new_deployment = true

  network_configuration {
    subnets          = var.subnets
    security_groups  = var.security_groups
    assign_public_ip = false
  }

  tags = {
    Name = var.db_name
    Type = "faaas-pg-proxy"
  }
}

resource "aws_ecs_task_definition" "pg_proxy" {
  family                   = "pg-proxy-${var.db_name}-task-def"
  execution_role_arn       = var.execution_role_arn
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  container_definitions = jsonencode([{
    name  = "pg-proxy-${var.db_name}-container-def"
    image = "${var.repository_url}:latest"
    environment = [
      {
        name  = "MQ_HOST"
        value = var.mq.host
      },
      {
        name  = "MQ_PORT"
        value = var.mq.port
      },
      {
        name  = "MQ_USER"
        value = var.mq.user
      },
      {
        name  = "MQ_PASS"
        value = var.mq.pass
      },
      {
        name  = "PG_HOST"
        value = var.pg.host
      },
      {
        name  = "PG_PORT"
        value = var.pg.port
      },
      {
        name  = "PG_USER"
        value = var.pg.user
      },
      {
        name  = "PG_PASS"
        value = var.pg.pass
      },
      {
        name  = "PG_DB"
        value = var.db_name
      }
    ]
    logConfiguration = {
      logDriver = "awslogs"
      options = {
        "awslogs-create-group" : "true"
        "awslogs-group" : "/aws/ecs/faaas-pg-proxy/${var.db_name}"
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
    Name = var.db_name
    Type = "faaas-pg-proxy-task-def"
  }
}

resource "aws_cloudwatch_log_group" "pg_proxy" {
  name = "/aws/ecs/faaas-pg-proxy/${var.db_name}"
}
