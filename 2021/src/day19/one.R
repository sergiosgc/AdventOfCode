#!/usr/bin/R --vanilla -f
library(purrr)
library(stringr)

parse_data <- function() {
  rawInput <- str_extract_all(readLines(file("stdin")), '-?[:digit:]+')
  scanners <- which(map(rawInput, length) == 1)
  scannerBeacons <- map(1:(length(scanners)), ~ unlist(rawInput[ (scanners[.] + 1):ifelse(. == length(scanners), length(rawInput), scanners[. + 1] - 1) ]) %>% map(as.numeric))
  return(map(scannerBeacons, matrix, nrow=3, dimnames=list(c("X","Y","Z"))))
}
beacon_distances <- function(beacons) matrix(ncol=dim(beacons)[2], apply(beacons, 2, function(LB) apply(beacons, 2, function(RB) sqrt( (RB[["X"]] - LB[["X"]])^2 + (RB[["Y"]] - LB[["Y"]])^2 + (RB[["Z"]] - LB[["Z"]])^2) )))
add_beacon <- function(beacons, newBeacon) {
  pos <- which( (beacons %>% 
    map(function(beacon) length(intersect(unlist(beacon), unlist(newBeacon)))) %>%
    unlist) >= 12)
  if (length(pos) > 0) map(pos, function(p) beacons[p] <- list(c(unlist(beacons[p]), unlist(newBeacon)) %>% unique) )
  else beacons[length(beacons) + 1] = list(newBeacon)
  return(beacons)
}
common_sensors <- function(A,B) {
  intersect_counts <- matrix(nrow=dim(A)[1], 
    unlist(map(1:dim(A)[1], function(a) map(1:dim(B)[1], function(b) length(intersect(A[a,], B[b,]))))), byrow=TRUE)
  search_for <- max(intersect_counts)
  if (search_for >= 12)
   result <- which(intersect_counts == search_for, arr.ind=TRUE)
  else result = matrix(ncol=2, c(0,0))
  colnames(result) <- c('a', 'b')
  return(result)
}
own_sensors <- function(my_distances, other_distances) {
  setdiff(1:dim(my_distances)[1], unlist(map(other_distances, ~ common_sensors(my_distances, .)[,'a'])))
}

scanners <- parse_data()
distances <- map(scanners, beacon_distances)
#map(1:(length(distances)-1), ~ own_sensors(distances[[(.)]], distances[(. + 1):length(distances)]))
unlist(map(1:(length(distances)-1), ~ own_sensors(distances[[(.)]], distances[(. + 1):length(distances)]) %>% length)) %>% sum + dim(distances[[length(distances)]])[1] - length(distances) - 1