task :default do
  sh "cargo +nightly fmt --all"
  sh "cargo build --examples --target wasm32-unknown-unknown --release"
  mkdir_p "target/examples"
  index = File.open("target/examples/index.html", "wb")
  FileList["examples/*.rs"].ext("").each do |name|
    name = File.basename(name)
    dir = "target/examples/#{name}"
    mkdir_p dir
    sh "wasm-bindgen target/wasm32-unknown-unknown/release/examples/#{name}.wasm --out-dir #{dir} --no-modules --no-modules-global Example"
    File.write "#{dir}/index.html", <<HTML
<main></main>
<script src="#{name}.js"></script>
<script>Example("#{name}_bg.wasm").then(() => Example.start());</script>
HTML
    index << <<HTML
<h3><a href="./#{name}/index.html">#{name}</a></h3>
HTML
  end

  cp "examples/jfb.json", "target/examples/jfb/package.json"
  cp "examples/jfb.html", "target/examples/jfb/index.html"
end
