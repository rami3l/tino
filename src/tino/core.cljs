(ns tino.core
  (:require [shadow.cljs.modern :refer (js-await)]
            ;; About the import trick below:
            ;; <https://gist.github.com/borkdude/7e548f06fbefeb210f3fcf14eef019e0>
            ["tio.js$default" :as tio]))

(defn main [& cli-args] (prn (.-languages tio)))

;; (defn tio-eval [code]
;;   (let [
;;         response (<p! )]))
