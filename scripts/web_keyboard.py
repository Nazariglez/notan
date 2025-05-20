
if __name__ == '__main__':
    import re
    import requests
    from sortedcontainers import SortedSet
    regex = re.compile(r'<td>\s*<code>"([0-9A-Za-z]+)"</code>')
    codes = SortedSet()
    data = requests.get('https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values').text
    for m in regex.finditer(data):
        code = m.groups()[0]
        if code != '' and code != 'Unidentified':
            codes.add(code)
    for code in codes:
        print(f'        "{code}" => KeyCode::{code},')
