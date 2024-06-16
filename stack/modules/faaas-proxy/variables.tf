variable "cluster_id" {
  type = string
}

variable "proxy_name" {
  type = string
}

variable "execution_role_arn" {
  type = string
}

variable "mq" {
  type = object({
    host = string
    port = string
    user = string
    pass = string
  })
}

variable "repository_url" {
  type = string
}

variable "subnets" {
  type = list(string)
}

variable "security_groups" {
  type = list(string)
}
