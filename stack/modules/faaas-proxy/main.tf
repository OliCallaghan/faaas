resource "aws_ecs_service" "proxy" {
  name                 = "${var.proxy_name}-proxy"
  cluster              = var.cluster_id
  task_definition      = aws_ecs_task_definition.proxy.arn
  desired_count        = 1
  launch_type          = "FARGATE"
  force_new_deployment = true

  network_configuration {
    subnets          = var.subnets
    security_groups  = var.security_groups
    assign_public_ip = false
  }

  tags = {
    Name = var.proxy_name
    Type = "faaas-proxy"
  }
}

resource "aws_ecs_task_definition" "proxy" {
  family                   = "${var.proxy_name}-proxy-task-def"
  execution_role_arn       = var.execution_role_arn
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  container_definitions = jsonencode([{
    name  = "${var.proxy_name}-proxy-container-def"
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
    ]
    logConfiguration = {
      logDriver = "awslogs"
      options = {
        "awslogs-create-group" : "true"
        "awslogs-group" : "/aws/ecs/faaas-${var.proxy_name}-proxy"
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
    Type = "faaas-${var.proxy_name}-proxy-task-def"
  }
}

resource "aws_cloudwatch_log_group" "proxy" {
  name = "/aws/ecs/faaas-${var.proxy_name}-proxy"
}
