import           Data.Char                      ( isSpace )
import           Data.List                      ( transpose )

type Line = String
type Numbers = [Int]
type Board = [[Int]]
type Boards = [Board]
type Marked = [Int]

data Bingo = Bingo Numbers Boards Marked
  deriving Show

splitString :: (Char -> Bool) -> String -> [String]
splitString f s = case dropWhile f s of
  "" -> []
  s' -> w : splitString f s'' where (w, s'') = break f s'

parseNumbers :: Line -> Numbers
parseNumbers = map read . splitString (== ',')

parseBoard :: [Line] -> Board
parseBoard = map (map read . words)

parseBoards :: [Line] -> Boards
parseBoards [] = []
parseBoards s  = parseBoard (take 5 s) : parseBoards (drop 5 s)

preprocessInput :: String -> [Line]
preprocessInput = filter (not . all isSpace) . lines

parseInput :: String -> Bingo
parseInput s = Bingo numbers boards []
 where
  numbers = parseNumbers (head input)
  boards  = parseBoards (tail input)
  input   = preprocessInput s

checkIfAnyRowCompleted :: Marked -> Board -> Bool
checkIfAnyRowCompleted marked = any (all (`elem` marked))

checkIfWins :: Marked -> Board -> Bool
checkIfWins marked board = anyRowCompleted || anyColumnCompleted
 where
  anyRowCompleted    = checkIfAnyRowCompleted marked board
  anyColumnCompleted = checkIfAnyRowCompleted marked (transpose board)

score :: Marked -> Board -> Int
score markedNumbers =
  (*) justCalledNumber . sum . filter (`notElem` markedNumbers) . concat
  where justCalledNumber = head markedNumbers

step :: ([Int] -> Int) -> Bingo -> (Bingo, Maybe Int)
step resolveTies (Bingo numbers boards marked) = (newState, newWinningScore)
 where
  newState        = Bingo (tail numbers) boards (head numbers : marked)
  winnerBoards    = filter (checkIfWins marked) boards
  newWinningScore = case winnerBoards of
    [] -> Nothing
    xs -> Just (resolveTies (map (score marked) winnerBoards))

stepMaximum :: Bingo -> (Bingo, Maybe Int)
stepMaximum = step maximum

stepMinimum :: Bingo -> (Bingo, Maybe Int)
stepMinimum = step minimum

dropWinnerBoards :: Bingo -> Bingo
dropWinnerBoards (Bingo numbers boards marked) = Bingo numbers
                                                       loserBoards
                                                       marked
  where loserBoards = filter (not . checkIfWins (tail marked)) boards

runBingo1 :: (Bingo, Maybe Int) -> Int
runBingo1 x@(bingo, Nothing) = runBingo1 (stepMaximum bingo)
runBingo1 (  _    , Just x ) = x

startBingo1 :: Bingo -> Int
startBingo1 bingo = runBingo1 (bingo, Nothing)

runBingo2 :: Maybe Int -> Bingo -> Int
runBingo2 (Just currentScore) (Bingo [] _ _) = currentScore
runBingo2 (Just currentScore) (Bingo _ [] _) = currentScore
runBingo2 currentScore bingo = runBingo2 newScore (dropWinnerBoards newBingo)
  where (newBingo, newScore) = stepMinimum bingo

startBingo2 :: Bingo -> Int
startBingo2 = runBingo2 Nothing

solvePart1 :: String -> Int
solvePart1 = startBingo1 . parseInput

solvePart2 :: String -> Int
solvePart2 = startBingo2 . parseInput

main :: IO ()
main = do
  input <- getContents
  print (solvePart1 input)
  print (solvePart2 input)
