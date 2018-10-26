task :default do
  filter = /#{ENV['filter'] || '.*'}/
  sh "cargo +nightly fmt --all"
  sh "cargo build --examples --target wasm32-unknown-unknown --release"
  mkdir_p "target/examples"
  index = File.open("target/examples/index.html", "wb")
  FileList["examples/*.rs"].ext("").grep(filter).each do |name|
    name = File.basename(name)
    dir = "target/examples/#{name}"
    mkdir_p dir
    sh "wasm-bindgen target/wasm32-unknown-unknown/release/examples/#{name}.wasm --out-dir #{dir} --no-modules --no-modules-global #{name}"
    File.write "#{dir}/index.html", <<HTML
<main></main>
<script src="#{name}.js"></script>
<script>#{name}("#{name}_bg.wasm").then(() => #{name}.start());</script>
HTML
    index << <<HTML
<h3><a href="./#{name}/index.html">#{name}</a></h3>
HTML
  end

  cp "examples/jfb.json", "target/examples/jfb/package.json"
  cp "examples/jfb.html", "target/examples/jfb/index.html"
end
