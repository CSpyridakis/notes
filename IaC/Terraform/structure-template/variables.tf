variable "example_name" {
  description = "The name of the example"
  type        = string
  default     = "Root Example"
}

variable "example_count" {
  description = "The number of resources"
  type        = number
  default     = 2
}

variable "example_map" {
  description = "A sample map variable"
  type        = map(string)
  default     = {
    "key1" = "value1"
    "key2" = "value2"
  }
}