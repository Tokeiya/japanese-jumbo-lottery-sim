
def pivot_longer():
    arr=[
        'first',
        'first_adjacent',
        'first_different_group',
        'second',
        'third',
        'fourth',
        'fifth',
        'sixth',
        'seventh'
    ]

    cnt=1

    tmp=[(i,f'{prize}_cnt',f'{prize}_prize') for (i,prize) in enumerate(arr,start=1)]

    for (i,c,p) in tmp:
        cmd=f"""INSERT INTO prizes(person_id, kind, cnt, payout) SELECT id,{i},{c},{p} FROM raw WHERE {c}<>0;"""
        print(cmd)

if __name__ == '__main__':
    pivot_longer()