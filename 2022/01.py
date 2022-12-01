with open("01.inp") as rf:
	data = [0]
	for line in rf.readlines():
		try:
			num = int(line.strip())
		except ValueError:
			data.append(0)
			continue
		data[-1] += num

	print(max(data))
	print(sum(sorted(data, reverse=True)[:3]))
