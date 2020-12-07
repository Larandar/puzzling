class pipe:
    def __init__(self, func) -> None:
        super().__init__()
        self.__inner_func = func

    def __call__(self, *args, **kwds):
        return self.__inner_func(*args, **kwds)

    def __or__(self, other):
        return map(self.__inner_func, other)


def split_on(part):
    @pipe
    def inner_func(str):
        k, _, v = str.partition(part)
        return k, v

    return inner_func
