import sqlite3
import matplotlib.pyplot as plt

# ============

connection = sqlite3.connect('./tests/results/database-wasm-small.db')
cursor = connection.cursor()

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'ready_time' LIMIT 100) value;")
result = cursor.fetchall()
wasm_ready_time = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'used_heap_size_start' LIMIT 100) value;")
result = cursor.fetchall()
wasm_used_heap_size_start = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'result_time' LIMIT 100) value;")
result = cursor.fetchall()
wasm_result_time = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'used_heap_size_end' LIMIT 100) value;")
result = cursor.fetchall()
wasm_used_heap_size_end = round(result[0][0], 2)

connection.close()

# ============

connection = sqlite3.connect('./tests/results/database-js-small.db')
cursor = connection.cursor()

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'ready_time' LIMIT 100) value;")
result = cursor.fetchall()
js_ready_time = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'used_heap_size_start' LIMIT 100) value;")
result = cursor.fetchall()
js_used_heap_size_start = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'result_time' LIMIT 100) value;")
result = cursor.fetchall()
js_result_time = round(result[0][0], 2)

cursor.execute("SELECT AVG(value) FROM (SELECT value FROM results WHERE type = 'used_heap_size_end' LIMIT 100) value;")
result = cursor.fetchall()
js_used_heap_size_end = round(result[0][0], 2)

connection.close()

# ============

figure, axis = plt.subplots(2, 2)

bar_names = ['JS', 'WASM']
data1 = [js_ready_time, wasm_ready_time]
data2 = [js_used_heap_size_start, wasm_used_heap_size_start]
data3 = [js_result_time, wasm_result_time]
data4 = [js_used_heap_size_end, wasm_used_heap_size_end]

bars = axis[0, 0].bar(bar_names, data1)
axis[0, 0].set_title("Время готовности, мс")
axis[0, 0].bar_label(bars, label_type='center')

bars = axis[0, 1].bar(bar_names, data2)
axis[0, 1].set_title("RAM на момент готовности, мб")
axis[0, 1].bar_label(bars, label_type='center')

bars = axis[1, 0].bar(bar_names, data3)
axis[1, 0].set_title("Время результата, мс")
axis[1, 0].bar_label(bars, label_type='center')

bars = axis[1, 1].bar(bar_names, data4)
axis[1, 1].set_title("RAM на момент результата, мб")
axis[1, 1].bar_label(bars, label_type='center')

plt.tight_layout()
plt.savefig('./tests/images/plot-100-small.png')
