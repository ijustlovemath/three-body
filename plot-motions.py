import matplotlib.pyplot as plt

with open('motion.csv', 'r') as f:
    lines = f.readlines()

def clean(s):
    for replacement in ['Some', '(', ')']:
        s = s.replace(replacement, '')
    return float(s)

def add_data_point(container, name, x, y):
    if name not in container:
        container[name] = lambda: None
        container[name].x = []
        container[name].y = []
    container[name].x.append(x)
    container[name].y.append(y)

data = {}
for line in lines:
    name, values = line.split(':')
    assert name in ['moon', 'earth']
    x, y = values.split(',')
    x = clean(x)
    y = clean(y)
    add_data_point(data, name, x, y)

for planet in data:
    plt.scatter(data[planet].x, data[planet].y, label=planet)

plt.legend(loc='upper right')
plt.show()