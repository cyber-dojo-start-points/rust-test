
lambda { |stdout,stderr,status|
  output = stdout + stderr
  return :red   if /test result: FAILED/.match(output)
  # cargo runs the lib, the integration tests and the doc-tests as separate
  # targets and prints a summary line for each, and a target holding no tests
  # still calls itself ok. Green needs one of those summaries to have passed
  # something: a kata with no tests, or with every test marked #[ignore],
  # arrives here with every passed count at zero and has proved nothing.
  return :green if /test result: ok\. [1-9]\d* passed/.match(output)
  return :amber
}
