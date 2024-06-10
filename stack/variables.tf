locals {
  build_path           = "node_modules/@faaas-bench/pets/dist"
  lambda_code_filename = "publishBookReview.zip"
  lambda_src_path      = "./src"

  gateway_log_group      = "/aws/ecs/faaas-gateway"
  proxy_sql_pg_log_group = "/aws/ecs/faaas-proxy-sql-pg"
}
