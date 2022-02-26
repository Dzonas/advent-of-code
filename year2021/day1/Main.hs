str2int :: String -> Int
str2int s = read s :: Int

parse :: String -> [Int]
parse content = map str2int (lines content)

pairs :: [a] -> [(a, a)]
pairs xs = zip xs (tail xs)

triples :: [a] -> [(a, a, a)]
triples xs = zip3 xs (tail xs) (tail (tail xs))

solve_p1 :: [Int] -> Int
solve_p1 ns = length (filter (\(a,b) -> b > a) (pairs ns))

solve_p2 :: [Int] -> Int
solve_p2 ns = solve_p1 (map (\(a,b,c) -> a + b + c) (triples ns))

main :: IO ()
main = do
    raw_input <- getContents
    let input = parse raw_input
    let p1_solution = solve_p1 input
    let p2_solution = solve_p2 input
    putStrLn (show p1_solution)
    putStrLn (show p2_solution)