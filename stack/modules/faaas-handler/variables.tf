variable "handler_name" {
  description = "FaaAS handler name"
  type        = string
}

variable "handler_zip" {
  description = "Path to the handler ZIP archive"
  type        = string
}

variable "rabbit_mq_arn" {
  description = "RabbitMQ ARN"
  type        = string
}

variable "rabbit_mq_secret_arn" {
  description = "RabbitMQ secret ARN"
  type        = string
}
# aws_secretsmanager_secret_version.rabbit_mq_auth.arn

variable "redis_security_group_id" {
  description = "Redis security group ID"
  type        = string
}

variable "subnet_ids" {
  description = "Subnet IDs"
  type        = set(string)
}
# [
#   aws_subnet.private_subnet_1.id,
#   aws_subnet.private_subnet_2.id,
# ]

variable "vpc_id" {
  description = "VPC ID where the Lambda function will be deployed"
  type        = string
}
# aws_vpc.main.id
