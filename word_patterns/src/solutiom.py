def word_pattern(word):
    counter = 0
    list_of_counter = []
    list_of_values = []


    for value in (word):
        value = value.lower()
        if value not in (list_of_values):
            list_of_counter.append(counter)
            list_of_values.append(value)
            counter += 1
        else:
            add_number = list_of_values.index(value)
            list_of_counter.append(add_number)

    result = '.'.join(map(str,list_of_counter))
    return (result)