locals {
  build_path           = "node_modules/@faaas-bench/pets/dist"
  lambda_code_filename = "publishBookReview.zip"
  lambda_src_path      = "./src"

  gateway_log_group      = "/aws/ecs/faaas-gateway"
  proxy_sql_pg_log_group = "/aws/ecs/faaas-proxy-sql-pg"

  rabbit_mq_host = "b-5a738d61-9443-4566-84e9-2c576774f230.mq.eu-west-2.amazonaws.com"
}
