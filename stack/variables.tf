locals {
  build_path           = "build"
  lambda_code_filename = "publishBookReview.zip"
  lambda_src_path      = "./src"

  gateway_log_group = "/aws/ecs/faaas-gateway"
}