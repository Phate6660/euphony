#!/usr/bin/env racket

#lang racket/base

(require racket/cmdline ffi/unsafe)

(define backend-lib (ffi-lib "backend/target/release/libbackend.so"))
(define backend-play (get-ffi-obj "play" backend-lib (_fun _string -> _void)))

(define audio-file (make-parameter 'file))

(command-line
  #:program "Euphony"
  #:once-any
  [("-p" "--play") str "Play the given file" (backend-play str)])
