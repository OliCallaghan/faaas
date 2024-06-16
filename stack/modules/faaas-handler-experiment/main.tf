module "strategy" {
  for_each = toset(["proxy", "local", "adaptive"])

  source = "../faaas-handler"

  handler_name = "${var.handler_name}-${each.key}"
  handler_zip  = var.handler_mq_zip

  monitor_image   = var.monitor_image
  monitor_enabled = each.key == "adaptive"
  monitor_rate    = var.monitor_rate

  vpc_id     = var.vpc_id
  subnet_ids = var.subnet_ids

  rabbit_mq_arn        = var.rabbit_mq_arn
  rabbit_mq_secret_arn = var.rabbit_mq_secret_arn

  redis_security_group_id = var.redis_security_group_id

  environment = {
    FAAAS_STRATEGY = each.key
  }
}

resource "aws_lambda_function" "http" {
  filename = var.handler_http_zip
  handler  = "handler.entrypoint"
  runtime  = "nodejs20.x"
  role     = aws_iam_role.http.arn

  memory_size = 256

  source_code_hash = filebase64sha256(var.handler_http_zip)

  function_name = "${var.handler_name}-http"
  timeout       = 30
}

resource "aws_iam_role" "http" {
  name = "faaas-experiment-handler-iam-${var.handler_name}-http"

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
    name = "cloudwatch_access"

    policy = jsonencode({
      "Version" : "2012-10-17",
      "Statement" : [
        {
          "Action" : [
            "ec2:CreateNetworkInterface",
            "ec2:DeleteNetworkInterface",
            "ec2:DescribeNetworkInterfaces",
            "ec2:DescribeSecurityGroups",
            "ec2:DescribeSubnets",
            "ec2:DescribeVpcs",
            "elasticache:Connect",
            "logs:CreateLogGroup",
            "logs:CreateLogStream",
            "logs:PutLogEvents",
            "logs:DescribeQueries",
            "logs:GetQueryResults",
            "logs:StartQuery"
          ],
          "Resource" : "*",
          "Effect" : "Allow"
        }
      ]
    })
  }
}

resource "aws_lambda_function_url" "http" {
  function_name      = aws_lambda_function.http.function_name
  authorization_type = "NONE"
}
