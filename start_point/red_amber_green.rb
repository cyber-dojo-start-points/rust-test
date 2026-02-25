
lambda { |stdout,stderr,status|
  output = stdout + stderr
  return :red   if /test result: FAILED/.match(output)
  return :green if /test result: ok/.match(output)
  return :amber
}
