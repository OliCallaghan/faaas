variable "handler_name" {
  description = "FaaAS handler name"
  type        = string
}

variable "handler_mq_zip" {
  description = "Path to the handler MQ ZIP archive"
  type        = string
}

variable "handler_http_zip" {
  description = "Path to the handler HTTP ZIP archive"
  type        = string
}

variable "memory_size" {
  description = "Memory size"
  type        = string
}

variable "monitor_enabled" {
  type    = bool
  default = false
}

variable "monitor_rate" {
  type    = string
  default = "5 minutes"
}

variable "monitor_image" {
  description = "Path to the monitor ZIP archive"
  type = object({
    repository_url = string
    version        = string
  })
}

variable "rabbit_mq_arn" {
  description = "RabbitMQ ARN"
  type        = string
}

variable "rabbit_mq_secret_arn" {
  description = "RabbitMQ secret ARN"
  type        = string
}

variable "redis_security_group_id" {
  description = "Redis security group ID"
  type        = string
}

variable "subnet_ids" {
  description = "Subnet IDs"
  type        = set(string)
}

variable "vpc_id" {
  description = "VPC ID where the Lambda function will be deployed"
  type        = string
}
