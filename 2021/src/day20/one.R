#!/usr/bin/R --vanilla -f
library(purrr)
library(stringr)

read_input <- function() {
  rawInput <- readLines(file("stdin"))
  algo <- unlist(map(strsplit(rawInput[1], split=""), function(c) ifelse(c=='#', 1, 0)))
  image <- map(rawInput[3:length(rawInput)], function(l) map(strsplit(l, split=""), function(c) ifelse(c=='#', 1, 0)))
  image <- matrix(byrow=1,nrow=length(image), unlist(image))
  return(list(algo, image))
}
pad_matrix <- function(m,pad,color) {
    result = matrix(color, nrow=dim(m)[1] + pad*2, ncol=dim(m)[2] + pad * 2)
    for (row in 1:dim(m)[1]) { for (col in 1:dim(m)[2]) { result[row+pad,col+pad] <- m[row,col] }}
    return(result)
}
enhancement_index <- function(m, r, c) {
    result = 0;
    for (row in (r-1):(r+1)) for (col in (c-1):(c+1)) result <- result * 2 + m[row,col]
    return(result)
}
convert_pixel <- function(algo, img, row, col) {
    result <- algo[[enhancement_index(img, row, col) + 1]]
    return(result)
}
enhance <- function(algo, source) {
    infinite_color <- source[1,1]
    img <- pad_matrix(source, 2, infinite_color)
    result <- matrix(infinite_color, nrow=dim(img)[1], ncol=dim(img)[2])
    for (row in 2:(dim(img)[1]-1)) for (col in 2:(dim(img)[2]-1)) result[row,col] = convert_pixel(algo, img, row, col)
    return(result[2:(dim(result)[1]-1),2:(dim(result)[2]-1)])
}
algo <- read_input()
image <- algo[[2]]
algo <- algo[[1]]
image <- pad_matrix(image, 1, 0)
image <- enhance(algo, image)
image <- enhance(algo, image)
print(length(which(c(image) == 1)))
for (i in 3:50) image <- enhance(algo, image)
print(length(which(c(image) == 1)))