import Data.List(sort, transpose)
import qualified Data.Map.Strict as Map

parseChar :: Char -> Int
parseChar '0' = 0
parseChar '1' = 1

parseLine :: String -> [Int]
parseLine = map parseChar

parseContents :: String -> [[Int]]
parseContents = map parseLine . lines

bin2int :: [Int] -> Int
bin2int ns = foldr (\x y -> x + 2 * y) 0 (reverse ns)

counter :: Ord k => [k] -> Map.Map k Int
counter = Map.fromListWith (+) . map (\x -> (x, 1))

maxValue :: Map.Map k Int -> Int
maxValue m = foldr max x xs
               where (x:xs) = Map.elems m
               
minValue :: Map.Map k Int -> Int
minValue m = foldr min x xs
               where (x:xs) = Map.elems m

mostCommon :: Map.Map k Int -> [k]
mostCommon m = map fst (filter (\(_,y) -> y == maxVal) (Map.assocs m))
  where maxVal = maxValue m

leastCommon :: Map.Map k Int -> [k]
leastCommon m = map fst (filter (\(_,y) -> y == minVal) (Map.assocs m))
  where minVal = minValue m

gammaRate :: [[Int]] -> [Int]
gammaRate = map (head . mostCommon . counter)

epsilonRate :: [[Int]] -> [Int]
epsilonRate =  map (head . leastCommon . counter)

solveP1 :: [[Int]] -> Int
solveP1 input = 
    g * e
  where
    t = transpose input
    g = bin2int (gammaRate t)
    e = bin2int (epsilonRate t)
    
oxygenGeneratorRating :: [[Int]] -> [Int]
oxygenGeneratorRating [[]] = []
oxygenGeneratorRating xss =
    mostCommonFirstBit : oxygenGeneratorRating ((dropHeads . keepRowsWithMostCommonFirstBit) xss)
  where
    mostCommonFirstBit = (last . sort . mostCommon . counter . head . transpose) xss
    dropHeads = map tail
    keepRowsWithMostCommonFirstBit = filter ((==mostCommonFirstBit) . head)
    
    
co2ScrubberRating :: [[Int]] -> [Int]
co2ScrubberRating [[]] = []
co2ScrubberRating xss =
    leastCommonFirstBit : co2ScrubberRating ((dropHeads . keepRowsWithLeastCommonFirstBit) xss)
  where
    leastCommonFirstBit = (head . sort . leastCommon . counter . head . transpose) xss
    dropHeads = map tail
    keepRowsWithLeastCommonFirstBit = filter ((==leastCommonFirstBit) . head)
    

solveP2 :: [[Int]] -> Int
solveP2 input = 
    ogr * csr
  where
    ogr = bin2int (oxygenGeneratorRating input)
    csr = bin2int (co2ScrubberRating input)

main :: IO ()
main = do
    contents <- getContents
    putStrLn (show (solveP1 (parseContents contents)))
    putStrLn (show (solveP2 (parseContents contents)))
    return ()