import sqlite3

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

connection = sqlite3.connect('./tests/results/database-js-small.db')
cursor = connection.cursor()

cursor.execute('''
CREATE TABLE IF NOT EXISTS results (
	id INTEGER PRIMARY KEY,
	type VARCHAR NOT NULL,
	value DOUBLE NOT NULL
)
''')

connection.commit()

chrome_options = Options()

chrome_options.add_argument("--headless")
chrome_options.add_argument("--disable-gpu")
chrome_options.add_argument("--no-sandbox")
chrome_options.add_argument("--disable-cache")
chrome_options.add_argument("--enable-precise-memory-info")

driver = webdriver.Chrome(options=chrome_options)

for i in range(0, 1000):
	print(i)
	# driver.get("http://localhost/wasm.html")
	driver.get("http://localhost/js.html")

	try:
		element = WebDriverWait(driver, 10).until(
			EC.text_to_be_present_in_element((By.ID, "ready"), "ready")
		)
	except:
		driver.quit()

	ready_element = driver.find_element(By.ID, "ready")
	ready_text = ready_element.get_attribute("innerHTML")
	(dummy, ready_time) = ready_text.split()
	# print(ready_text)

	used_heap_size = driver.execute_script("return window.performance.memory.usedJSHeapSize;")
	used_heap_size_start = used_heap_size / (1024 * 1024)
	# print(f"memory start {used_heap_size_start}")

	field_element = driver.find_element(By.ID, "file-input")
	field_element.send_keys('/home/ihor/cat.jpg')

	try:
		element = WebDriverWait(driver, 10).until(
			EC.text_to_be_present_in_element((By.ID, "result"), "result")
		)
	except:
		driver.quit()

	result_element = driver.find_element(By.ID, "result")
	result_text = result_element.get_attribute("innerHTML")
	(dummy, result_time, dummy2) = result_text.split()
	# print(result_text)

	used_heap_size = driver.execute_script("return window.performance.memory.usedJSHeapSize;")
	used_heap_size_end = used_heap_size / (1024 * 1024)
	# print(f"memory end {used_heap_size_end}mb")

	original_tab = driver.current_window_handle
	driver.switch_to.new_window('tab')
	new_tab = driver.current_window_handle
	driver.switch_to.window(original_tab)
	driver.close()
	driver.switch_to.window(new_tab)

	cursor.execute('INSERT INTO results (type, value) VALUES (?, ?)', ('ready_time', ready_time))
	cursor.execute('INSERT INTO results (type, value) VALUES (?, ?)', ('used_heap_size_start', used_heap_size_start))
	cursor.execute('INSERT INTO results (type, value) VALUES (?, ?)', ('result_time', result_time))
	cursor.execute('INSERT INTO results (type, value) VALUES (?, ?)', ('used_heap_size_end', used_heap_size_end))

	connection.commit()

driver.quit()