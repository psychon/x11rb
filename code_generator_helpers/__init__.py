import re


def camel_case_to_lower_snake(arg):
    # Based on _cname_re from libxcb's c_client.py
    pattern = re.compile('([A-Z][a-z0-9]+|[A-Z]+(?![a-z0-9])|[a-z0-9]+)')
    split = pattern.finditer(arg)
    arg_parts = [match.group(0) for match in split]
    return '_'.join(arg_parts).lower()


def camel_case_to_upper_snake(arg):
    return camel_case_to_lower_snake(arg).upper()
