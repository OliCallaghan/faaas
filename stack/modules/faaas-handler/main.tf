resource "aws_security_group" "faaas_handler" {
  name   = "faaas-handler-sg-${var.handler_name}"
  vpc_id = var.vpc_id

  ingress {
    from_port       = 0
    to_port         = 0
    protocol        = "-1"
    security_groups = [var.redis_security_group_id]
    cidr_blocks     = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = var.handler_name
    Type = "faaas-handler"
  }
}

resource "aws_lambda_function" "faaas_handler" {
  filename = var.handler_zip
  handler  = "handler.entrypoint"
  runtime  = "nodejs20.x"
  role     = aws_iam_role.faaas_handler.arn

  memory_size = 256

  source_code_hash = filebase64sha256(var.handler_zip)

  function_name = var.handler_name
  timeout       = 30

  environment {
    variables = merge({
      MQ_HOST   = "b-5a738d61-9443-4566-84e9-2c576774f230.mq.eu-west-2.amazonaws.com"
      MQ_PORT   = "5671"
      MQ_USER   = "admin"
      MQ_PASS   = "ishouldmakethissecure"
      MQ_QUEUE  = "${var.handler_name}-queue"
      REDIS_URL = "rediss://redis-av1ecw.serverless.euw2.cache.amazonaws.com:6379"
    }, var.environment)
  }

  vpc_config {
    security_group_ids = [aws_security_group.faaas_handler.id]
    subnet_ids         = var.subnet_ids
  }
}

resource "aws_lambda_event_source_mapping" "handler_rabbit_mq_event_source" {
  batch_size       = 1
  event_source_arn = var.rabbit_mq_arn
  enabled          = true
  function_name    = aws_lambda_function.faaas_handler.arn
  queues           = ["${var.handler_name}-queue"]

  source_access_configuration {
    type = "VIRTUAL_HOST"
    uri  = "/"
  }

  source_access_configuration {
    type = "BASIC_AUTH"
    uri  = var.rabbit_mq_secret_arn
  }
}

resource "aws_iam_role" "faaas_handler" {
  name = "faaas-handler-iam-${var.handler_name}"

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
