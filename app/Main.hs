module Main where

-- import Day01 (part1, part2)
import Day02 (part1)

main :: IO ()
main = do
    input <- readFile "input/day02.txt"
    putStrLn $ "Part 1: " ++ (show $ part1 input)
    putStrLn $ "Part 2: "
