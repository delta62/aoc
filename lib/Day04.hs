module Day04 where

import Data.List (isPrefixOf, transpose)
import qualified Data.Set as S
import Util (count)

rows :: String -> [String]
rows = lines

cols :: String -> [String]
cols = transpose . rows

rotate90 :: [[a]] -> [[a]]
rotate90 = reverse . transpose

rotate180 :: [[a]] -> [[a]]
rotate180 = rotate90 . rotate90

diagonals :: [[a]] -> [[a]]
diagonals =
  (++)
    <$> transpose . zipWith drop [0 ..]
    <*> transpose . zipWith drop [1 ..] . rotate180

diagonals' :: [[a]] -> [[a]]
diagonals' = diagonals . rotate90

countOccurrences :: (Eq a) => [a] -> [a] -> Int
countOccurrences [] _ = 0
countOccurrences _ [] = 0
countOccurrences test xs =
  if test `isPrefixOf` xs
    then 1 + countOccurrences test rest
    else countOccurrences test (tail xs)
  where
    rest = drop (length test) xs

isCross :: [String] -> (Int, Int) -> Bool
isCross grid (x, y) = mid == 'A' && S.fromList [nw, se] == ms && S.fromList [ne, sw] == ms
  where
    ms = S.fromList "MS"
    mid = grid !! y !! x
    nw = grid !! pred y !! pred x
    ne = grid !! pred y !! succ x
    sw = grid !! succ y !! pred x
    se = grid !! succ y !! succ x

part1 :: String -> Int
part1 input =
  sum $ forwardMatches ++ backwardMatches
  where
    rs = rows input
    allDirections = concat [cols input, rs, diagonals rs, diagonals' rs]
    forwardMatches = map (countOccurrences "XMAS") allDirections
    backwardMatches = map (countOccurrences "SAMX") allDirections

part2 :: String -> Int
part2 input = count check points
  where
    maxX = length (head $ rows input) - 2
    maxY = length (rows input) - 2
    points = [(x, y) | x <- [1 .. maxX], y <- [1 .. maxY]]
    check = isCross $ rows input
