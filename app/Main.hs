module Main where

-- import Day01 (part1, part2)
-- import Day02 (part1, part2)
import Day03 (part1, part2)

-- import Day04 (part1)

main :: IO ()
main = do
  input <- readFile "input/day03.txt"
  putStrLn $ "Part 1: " ++ show (part1 input)
  putStrLn $ "Part 2: " ++ show (part2 input)
