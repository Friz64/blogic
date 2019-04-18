input=shaders/
output=assets/shaders/

rm -r $output/*

for path in $input/*.vert; do
    name=$(basename "$path")
    echo "===> Compiling vertex shader: $name"

    glslc -O -o $output/"$name".spv -fshader-stage=vert "$path"
done

echo ""

for path in $input/*.frag; do
    name=$(basename "$path")
    echo "===> Compiling fragment shader: $name"

    glslc -O -o $output/"$name".spv -fshader-stage=frag "$path"
done
