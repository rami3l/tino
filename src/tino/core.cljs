(ns tino.core
  (:require [cljs.nodejs :as nodejs]))

(nodejs/enable-util-print!)

(println "Hello world!")
