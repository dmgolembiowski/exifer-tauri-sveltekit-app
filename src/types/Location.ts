export interface Location {
  root: string,
  parsed: boolean,
  statistics?: LocationStatistics,
}

export interface LocationStatistics {
  totalFiles: number,
  parsableFiles: number,
  largestImageSize: number,
}