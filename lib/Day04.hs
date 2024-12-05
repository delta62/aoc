module Day04 where

import Data.List (isPrefixOf, transpose)

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

part1 :: String -> Int
part1 input =
  sum $ forwardMatches ++ backwardMatches
  where
    rs = rows input
    allDirections = concat [cols input, rs, diagonals rs, diagonals' rs]
    forwardMatches = map (countOccurrences "XMAS") allDirections
    backwardMatches = map (countOccurrences "SAMX") allDirections

part2 :: String -> Int
part2 = undefined
