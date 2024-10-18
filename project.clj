(defproject tino "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [org.clojure/clojurescript "1.11.132"]]
  :plugins [[lein-cljsbuild "1.1.8"]]
  :source-paths ["src"]
  :hooks [leiningen.cljsbuild]
  :cljsbuild {:builds [{:id "tino",
                        :source-paths ["src"],
                        :compiler {:warnings true,
                                   :optimizations :simple,
                                   ;; :output-to "index.js"
                                   :target :nodejs}}]})
