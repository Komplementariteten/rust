struct DataBuf {
    rows: u32,
    cols: u32,
    data: array<f32>,
}

@group(0)
@binding(0)
var<storage, read> input_a: DataBuf;

@group(0)
@binding(1)
var<storage, read> input_b: DataBuf;

@group(0)
@binding(2)
var<storage, read_write> output: DataBuf;

// Pass 0 = j
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // i = x
    // k = y
    if (output.rows != input_b.cols) {
        return;
    }
    if (output.cols != input_a.rows) {
        return;
    }
    var result: f32 = 0.0;
    // Rows
    var k: u32 = global_id.y;
    // Cols
    var i: u32 = global_id.x;
    for (var j: u32 = u32(0); j < input_a.cols; j++) {
        result = result + input_a.data[j + (j*i)] + input_b.data[j + (j * k)];
    }
    output[i + (i * k)] = result;
}