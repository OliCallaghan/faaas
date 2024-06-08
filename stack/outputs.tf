output "gateway_url" {
  description = "FaaAS Gateway URL"
  value       = "https://faaas-gateway-url-here.com"
}

output "rabbit_arn" {
  description = "ARN of the RabbitMQ broker."
  value       = aws_mq_broker.rabbit_mq.arn
}

output "rabbit_id" {
  description = "ID of the RabbitMQ broker."
  value       = aws_mq_broker.rabbit_mq.id
}

output "rabbit_console_url" {
  description = "The URL of the broker's RabbitMQ Web Console"
  value       = aws_mq_broker.rabbit_mq.instances.0.console_url
}

output "rabbit_ip_address" {
  description = "IP Address of the RabbitMQ broker"
  value       = aws_mq_broker.rabbit_mq.instances.0.ip_address
}

output "rabbit_endpoint" {
  description = "Broker's wire-level protocol endpoint"
  value       = aws_mq_broker.rabbit_mq.instances.0.endpoints
}
