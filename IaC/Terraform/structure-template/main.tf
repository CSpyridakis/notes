module "dummy" {
  source        = "./modules/dummy"
  example_name  = var.example_name
  example_count = var.example_count
}
