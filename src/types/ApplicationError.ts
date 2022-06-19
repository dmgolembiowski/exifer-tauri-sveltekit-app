export type ApplicationError =
  { UnsupportedFileType: [string, string] }
  | { UnknownFileType: [string, string] }
  | { LibraryParseError: [string, string] }
  | { ExifParseError: [string, string] }
  | { NoFileName: [string, string] }
  | { IOError: [string, string] };