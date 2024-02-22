# nolint start

#' @docType package
#' @usage NULL
#' @useDynLib boilerplateR, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
hello_world <- function() .Call(wrap__hello_world)
ex_robj_types <- function(val, arg = 1) .Call(wrap__ex_robj_types, val, arg)
kwargs <- function(...) .Call(wrap__kwargs, eval(substitute(alist(...))))
panic <- function(src) .Call(wrap__panic, src)

# nolint end
