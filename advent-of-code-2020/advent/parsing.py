def partition_on(part):
    def inner_func(str):
        k, _, v = str.partition(part)
        return k, v

    return inner_func
