import onnx, numpy as np
from onnx import helper, TensorProto
from pathlib import Path

X = helper.make_tensor_value_info('input',  TensorProto.FLOAT, [1, 256])
Y = helper.make_tensor_value_info('output', TensorProto.FLOAT, [1, 2])

W = np.random.randn(256, 2).astype(np.float32)
B = np.random.randn(2).astype(np.float32)
w_init = helper.make_tensor('W', TensorProto.FLOAT, [256, 2], W.tobytes(), raw=True)
b_init = helper.make_tensor('B', TensorProto.FLOAT, [2], B.tobytes(),  raw=True)

node  = helper.make_node('Gemm', ['input', 'W', 'B'], ['output'])
graph = helper.make_graph([node], 'dummy_model', [X], [Y], [w_init, b_init])

model = helper.make_model(
    graph,
    ir_version = 10,      # ORT 1.22 destekli
    opset_imports = [helper.make_opsetid("", 17)]   # <=22 olsun
)

dst = Path('ai-engine/ai_engine/models/malware_classifier.onnx')
dst.parent.mkdir(parents=True, exist_ok=True)
onnx.save(model, dst)
print(f'✓ model kaydedildi → {dst.resolve()} (IR={model.ir_version}, opset={model.opset_import[0].version})')