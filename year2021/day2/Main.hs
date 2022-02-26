str2int :: String -> Int
str2int s = read s :: Int

parse_line :: String -> (String, Int)
parse_line line = (a, str2int b)
  where [a,b] = words line

parse_content :: String -> [(String, Int)]
parse_content content = map parse_line (lines content)

run_command_p1 :: (Int, Int) -> (String, Int) -> (Int, Int)
run_command_p1 (y,x) (direction,n) | direction == "forward" = (y, x + n)
                                | direction == "down" = (y + n, x)
                                | direction == "up" = (y - n, x)
                                | otherwise = error "Unknown direction"

run_command_p2 :: (Int, Int, Int) -> (String, Int) -> (Int, Int, Int)
run_command_p2 (y, x, a) (direction, n)
    | direction == "forward" = (y + (a * n), x + n, a)
    | direction == "down" = (y, x, a + n)
    | direction == "up" = (y, x, a - n)
    | otherwise = error "Unknown direction"

solve_p1 :: [(String, Int)] -> Int
solve_p1 commands = y * x
  where (y, x) = foldl run_command_p1 (0, 0) commands

solve_p2 :: [(String, Int)] -> Int
solve_p2 commands = y * x
  where (y, x, _) = foldl run_command_p2 (0, 0, 0) commands

main :: IO ()
main = do
  content <- getContents
  let commands = parse_content content
  let p1_solution = solve_p1 commands
  let p2_solution = solve_p2 commands
  putStrLn (show p1_solution)
  putStrLn (show p2_solution)