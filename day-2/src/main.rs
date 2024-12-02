use std::str::FromStr;

fn increasing(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] > 3 || arr[i] <= arr[i - 1] {
            return false;
        }
    }

    return true;
}

fn decreasing(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] < -3 || arr[i - 1] <= arr[i] {
            return false;
        }
    }

    return true;
}

fn res_func(arr: &[i32]) -> bool {
    if arr[0] < arr[1] && arr[1] < arr[2] {
        return increasing(arr);
    } else if arr[0] > arr[1] && arr[1] > arr[2] {
        return decreasing(arr);
    } else {
        return false;
    }
}

fn main() {
    let input = "65 67 70 72 74 73
32 35 37 39 39
28 31 34 35 38 39 43
51 54 56 58 59 60 63 70
77 78 80 81 80 82
28 31 32 35 32 31
41 43 42 44 45 45
41 42 39 41 42 44 48
14 15 13 14 17 20 21 26
24 25 27 27 28 29 31 32
36 39 41 42 43 43 45 44
80 82 84 84 84
74 75 78 78 80 84
12 14 14 17 20 26
40 41 42 43 44 48 51 52
68 70 72 73 77 79 82 79
70 71 75 77 80 80
7 10 14 17 20 24
26 29 33 34 36 38 41 47
15 16 23 25 26 28 29
85 88 93 94 97 99 97
33 34 39 42 44 46 47 47
77 79 86 88 91 92 94 98
24 26 28 34 36 43
52 50 53 54 57 59 60
54 52 54 57 60 57
80 77 80 83 85 87 87
75 72 75 77 79 83
48 46 49 50 51 52 58
12 9 11 13 11 14 16
62 59 56 58 61 58
28 26 29 30 31 29 29
41 40 41 44 41 42 44 48
57 54 56 54 59
61 59 60 62 62 63 65
43 41 41 43 45 44
76 74 77 80 80 80
32 31 34 37 40 40 44
88 86 87 88 91 91 98
57 56 59 62 66 67 69 71
21 19 23 24 27 28 27
85 82 85 89 89
38 37 41 42 45 48 50 54
80 79 83 86 92
83 82 88 90 92 94
73 71 72 78 75
61 60 67 70 71 71
80 77 78 79 84 87 90 94
8 7 9 16 17 23
53 53 56 57 59 60 63
41 41 44 45 46 44
50 50 52 53 56 57 58 58
37 37 40 41 43 44 48
14 14 16 18 20 25
89 89 92 89 90 91
29 29 26 29 32 31
70 70 67 69 70 70
68 68 70 72 71 73 77
10 10 9 11 18
77 77 80 83 84 84 86
40 40 40 41 42 41
71 71 73 74 74 75 77 77
78 78 81 83 83 87
4 4 5 7 10 10 16
50 50 51 54 56 60 62 64
10 10 12 16 14
24 24 25 26 30 32 32
19 19 23 24 25 29
59 59 62 64 68 70 77
39 39 41 48 51 52
36 36 43 45 48 51 49
52 52 54 59 61 62 62
45 45 52 55 58 62
9 9 10 13 19 20 26
54 58 61 64 66
77 81 83 85 83
67 71 72 74 77 80 83 83
52 56 58 61 63 64 67 71
14 18 20 22 23 29
38 42 44 42 45 48 51
23 27 28 26 29 32 34 32
83 87 89 86 86
15 19 22 21 25
13 17 19 21 20 26
56 60 60 63 64
1 5 8 8 9 12 13 11
82 86 89 89 89
67 71 72 72 76
64 68 68 71 74 80
53 57 58 61 65 68 69 71
42 46 49 50 54 57 58 57
11 15 19 21 21
59 63 66 67 71 75
7 11 12 14 18 20 26
73 77 83 85 86 88 91 94
33 37 43 46 44
29 33 35 40 40
6 10 11 14 19 23
71 75 77 84 89
63 70 71 74 75 76 79 80
66 73 74 76 78 80 79
7 13 16 18 19 21 21
32 38 41 42 46
3 10 12 13 14 21
68 73 71 74 75
34 41 40 41 42 39
78 84 87 90 88 91 91
80 85 83 86 90
51 58 61 59 60 66
44 49 49 52 54
46 51 51 53 54 55 56 53
87 93 94 97 97 99 99
26 33 35 37 39 39 43
10 17 19 19 24
31 38 42 44 45
10 17 20 21 23 27 28 26
75 82 85 89 89
46 52 53 57 60 61 63 67
65 72 76 79 84
69 76 77 78 81 87 90 91
42 47 54 55 58 61 58
41 47 49 52 55 60 60
68 74 75 76 78 83 87
69 74 81 83 89
48 46 44 43 42 43
48 47 45 44 44
63 61 60 58 55 51
39 36 35 32 30 29 27 20
53 52 51 52 51 49 47 44
41 39 41 39 41
79 78 75 77 74 73 72 72
63 61 60 58 61 60 59 55
43 40 41 38 37 35 30
32 31 31 29 26
98 96 93 92 92 89 92
87 84 84 83 80 80
28 27 24 24 21 17
84 82 80 80 75
26 24 22 21 17 14 13
44 41 37 36 35 33 30 31
23 22 20 16 14 11 11
67 64 62 58 54
57 56 55 52 48 43
23 21 18 15 13 8 6 5
52 50 48 46 41 40 37 39
88 85 82 76 76
42 39 36 31 27
67 66 64 61 56 54 48
94 95 92 89 87 86 84
78 81 79 78 75 74 75
75 76 73 71 71
74 75 72 69 68 64
71 72 69 68 66 65 62 55
30 33 35 33 32
55 56 55 58 56 53 50 53
47 48 50 48 48
37 39 38 41 37
71 72 70 72 69 62
48 50 47 47 45
42 43 40 40 39 38 40
92 95 92 92 89 88 88
52 55 55 53 52 48
85 86 85 85 79
38 40 38 36 32 30 28 27
50 52 48 46 45 44 45
65 67 66 65 61 59 59
55 56 52 49 48 47 43
28 29 25 24 17
66 67 64 62 60 55 52
42 44 43 36 33 32 30 31
46 48 45 42 39 34 34
69 71 66 65 61
74 76 73 66 60
17 17 16 14 11 8
61 61 58 56 57
64 64 62 61 60 59 57 57
58 58 57 55 52 51 50 46
98 98 96 93 92 91 90 85
60 60 57 54 53 55 54
28 28 25 24 27 26 27
2 2 1 3 3
17 17 14 16 13 12 10 6
57 57 55 54 52 54 51 45
74 74 73 73 71
85 85 83 83 85
79 79 77 77 77
75 75 75 74 73 71 67
31 31 31 30 24
57 57 54 50 48 45 44
27 27 25 21 22
56 56 54 52 49 45 45
91 91 89 85 83 80 78 74
82 82 78 75 68
64 64 61 58 57 51 49 46
46 46 44 42 35 33 35
58 58 57 51 49 47 47
34 34 33 32 27 26 22
25 25 22 17 15 9
24 20 18 15 13 11
62 58 55 53 52 51 52
97 93 91 90 88 87 87
45 41 39 38 34
79 75 73 70 64
15 11 10 12 11 10 7
19 15 18 17 14 17
94 90 89 91 91
23 19 17 16 14 13 16 12
73 69 68 66 63 66 65 59
85 81 79 79 78 77
15 11 8 8 9
73 69 69 66 66
83 79 79 78 75 74 70
66 62 60 60 55
47 43 41 37 35 33
96 92 91 87 84 82 84
16 12 10 6 4 4
38 34 31 27 23
50 46 44 40 33
98 94 93 91 85 82 79
71 67 65 63 57 56 57
61 57 56 51 49 47 47
40 36 33 26 22
43 39 36 35 28 27 21
51 45 42 40 38
97 92 89 86 87
79 74 73 72 72
74 68 65 64 60
30 24 22 20 14
95 90 93 91 88
54 47 49 46 43 46
47 41 38 41 41
74 69 67 64 67 63
23 16 19 18 16 15 12 7
43 36 35 32 32 31 28
26 19 18 18 20
88 83 82 81 81 79 79
78 73 71 70 67 67 63
71 66 63 60 60 54
29 24 22 18 16 15 12
39 32 30 28 25 21 23
64 59 58 54 51 49 47 47
26 21 18 16 14 10 7 3
29 22 21 18 15 11 5
75 70 68 61 60
56 50 48 41 38 36 35 38
41 34 31 28 22 22
38 32 26 24 23 21 18 14
96 90 83 80 77 70
38 41 44 45 46 47 46
31 33 36 38 39 40 40
10 11 12 15 18 21 23 27
67 70 73 75 81
89 91 93 96 93 94 96 99
35 36 38 35 34
7 10 12 11 14 17 20 20
24 26 27 29 31 30 34
66 68 71 68 73
12 15 15 16 17 19
76 77 77 79 81 79
88 89 91 91 94 97 97
74 76 76 79 83
90 91 92 92 97
62 63 67 70 71 74 76 77
80 81 83 86 89 93 96 94
11 13 15 17 18 22 22
27 28 29 30 34 38
7 8 11 14 18 25
20 23 25 32 34
67 68 70 73 78 80 83 81
14 15 17 24 26 26
32 33 35 38 44 45 47 51
20 21 22 23 26 32 33 40
46 43 45 48 51
15 13 15 16 18 19 21 19
45 43 46 47 48 51 53 53
49 47 48 49 52 54 55 59
45 44 46 47 50 55
32 31 30 32 34 35 36
48 45 43 46 49 52 49
12 11 8 10 12 12
65 62 61 63 64 67 69 73
45 42 44 43 46 52
36 35 35 37 40
64 61 63 63 65 62
2 1 4 4 6 6
14 12 12 14 16 17 20 24
49 46 49 50 50 57
75 73 77 79 82
48 46 47 51 49
11 10 14 17 17
8 7 8 12 15 19
43 40 41 45 47 50 51 57
34 33 34 36 41 42 43
6 4 6 13 11
7 6 9 16 19 19
19 17 22 24 27 28 30 34
12 9 15 16 22
55 55 58 60 63
65 65 67 69 71 73 70
59 59 62 63 65 67 68 68
39 39 42 43 45 48 52
13 13 16 17 24
64 64 61 62 63
73 73 71 72 74 77 78 75
52 52 51 52 52
76 76 73 75 79
25 25 23 24 26 33
82 82 82 84 85 86
22 22 24 24 21
33 33 36 38 38 38
50 50 50 53 55 56 57 61
65 65 67 67 69 70 77
13 13 17 20 21
42 42 43 47 50 47
47 47 50 54 54
8 8 12 14 16 18 22
1 1 4 7 11 12 15 20
49 49 50 51 57 59
40 40 43 49 47
16 16 17 22 23 23
23 23 26 31 33 37
71 71 78 81 82 83 89
66 70 72 74 77 79 81 83
38 42 45 48 45
10 14 17 19 19
72 76 77 80 81 85
49 53 54 57 64
85 89 91 92 89 91
39 43 42 44 47 46
13 17 15 17 19 19
44 48 51 49 50 54
29 33 31 32 39
1 5 7 10 11 11 12 15
56 60 63 66 66 65
58 62 65 65 66 69 69
6 10 10 13 17
59 63 64 67 67 70 75
22 26 28 32 33
67 71 72 76 75
61 65 66 70 71 72 72
64 68 72 73 76 77 81
80 84 85 88 92 97
22 26 33 36 39 41 42 43
23 27 30 33 38 39 38
78 82 83 90 93 94 96 96
39 43 46 51 54 58
48 52 53 54 60 65
41 47 49 51 53
70 76 79 80 77
75 82 85 88 89 90 90
25 32 35 38 41 44 45 49
18 23 24 25 27 32
53 58 61 62 64 61 64
66 71 74 72 71
9 14 13 15 18 18
51 56 58 61 62 65 62 66
83 89 90 91 89 92 99
11 18 18 19 22
51 56 57 59 62 62 59
91 98 98 99 99
46 51 52 52 54 58
62 67 68 68 75
41 47 50 54 55 56
5 10 12 16 17 16
6 12 16 18 19 19
45 50 53 55 59 62 65 69
17 22 25 28 32 34 41
62 67 68 74 77 79
83 88 95 97 98 95
83 89 90 97 98 98
27 32 34 35 37 42 46
33 38 43 45 47 54
14 13 12 11 10 7 5 8
71 69 68 66 64 64
30 28 27 24 21 17
55 54 51 48 46 45 44 37
74 73 71 73 71
53 50 49 48 49 47 45 48
68 66 64 65 63 62 62
49 46 44 47 43
80 78 77 76 78 77 72
44 41 40 38 38 37
11 10 9 7 7 5 8
85 83 82 82 81 81
61 60 57 55 55 51
99 96 95 92 91 91 90 85
83 80 76 74 72
17 15 11 8 6 4 6
45 43 42 40 38 37 33 33
73 71 69 65 63 59
57 54 51 47 45 38
49 47 45 40 38 35
48 46 44 38 35 37
42 39 36 31 30 30
38 37 30 28 24
36 33 30 24 19
77 79 78 77 74 72 69 66
14 16 13 12 10 7 10
48 51 48 45 45
70 71 70 69 65
93 95 94 93 91 84
10 13 11 9 12 9
23 24 22 23 24
67 70 67 68 66 64 64
77 79 81 80 76
18 20 17 14 13 11 13 6
42 45 45 42 40 37 34 32
63 66 65 63 63 61 64
39 42 40 39 39 39
80 81 80 80 76
45 48 48 47 46 45 38
58 60 56 55 53 52
29 30 28 26 22 23
33 34 30 29 28 28
88 89 88 84 82 78
70 73 70 66 64 61 55
69 72 67 65 64 61 58
88 89 88 86 80 83
23 26 24 23 16 16
50 51 50 44 40
66 67 66 65 63 56 55 49
49 49 46 44 43
21 21 18 15 13 11 9 10
59 59 58 56 56
58 58 56 54 50
69 69 67 65 63 58
32 32 34 31 29
41 41 43 40 42
66 66 63 60 63 60 60
30 30 29 30 27 26 23 19
55 55 58 56 50
26 26 25 25 24
48 48 46 43 43 41 38 39
31 31 30 30 30
28 28 25 25 23 20 16
52 52 49 46 46 39
83 83 81 77 74
43 43 41 39 36 32 33
80 80 78 75 71 71
52 52 48 47 45 41
65 65 64 61 60 56 53 48
78 78 75 73 70 68 62 60
35 35 30 29 28 31
42 42 41 36 36
85 85 84 82 75 72 69 65
59 59 57 54 49 44
50 46 44 41 39 38 37
42 38 35 34 32 34
37 33 32 29 27 26 26
40 36 35 32 28
80 76 74 71 69 67 64 59
97 93 90 92 91
19 15 13 16 18
65 61 62 59 59
60 56 53 55 53 51 47
37 33 31 29 32 25
62 58 57 56 55 55 52
97 93 92 90 89 89 87 89
55 51 48 48 46 46
38 34 34 31 30 26
20 16 16 15 12 10 9 3
63 59 58 54 51
38 34 30 28 27 26 28
43 39 38 34 34
57 53 52 50 46 45 42 38
34 30 29 25 22 20 19 12
86 82 76 74 73 70 67
50 46 41 39 40
60 56 54 52 46 45 45
57 53 52 50 48 43 41 37
90 86 83 80 73 66
85 80 78 75 72 71 68
78 72 69 68 70
70 65 63 62 60 57 54 54
78 72 69 67 66 64 60
40 34 31 30 29 22
79 74 77 76 73 70 68
20 13 11 14 15
12 6 4 3 4 3 3
52 45 44 46 44 43 42 38
54 47 44 42 41 42 39 32
90 83 81 79 78 78 76 74
95 90 88 85 84 84 82 85
22 16 15 14 14 14
97 90 89 86 84 81 81 77
28 22 22 20 13
49 44 42 41 37 34 32 29
30 25 21 20 23
57 51 48 44 43 40 40
64 58 54 51 49 45
62 55 51 49 42
34 29 26 23 21 16 13 12
48 42 41 40 33 31 30 33
95 90 85 82 80 80
77 71 69 62 58
70 63 58 55 53 46
30 31 32 35 36 39 36
82 84 86 88 91 91
75 77 80 83 87
76 79 82 84 87 92
38 40 43 46 44 45 47
92 94 97 94 92
53 54 51 53 53
6 7 4 6 8 12
34 35 37 36 43
48 51 54 54 56 59 61 64
12 14 14 17 19 21 20
21 22 24 25 27 27 27
68 69 69 70 74
2 4 5 7 7 12
60 61 63 67 69
40 41 44 48 50 48
32 33 36 38 42 43 44 44
64 67 71 74 77 80 84
38 41 45 47 49 55
17 20 22 24 26 32 34 35
82 85 87 92 93 95 94
54 55 61 64 64
38 41 48 49 50 52 56
65 68 71 76 77 78 79 84
52 51 53 55 57 58 59 61
9 6 7 8 10 11 13 12
45 44 45 46 46
37 36 37 38 42
87 85 87 90 97
91 88 89 86 89
50 48 49 48 51 53 56 55
45 43 41 42 43 44 44
74 72 75 73 77
22 21 22 21 22 25 31
36 35 37 39 42 42 45
78 77 77 79 82 85 83
66 65 68 70 72 72 74 74
28 25 25 28 30 31 35
75 72 72 74 77 84
20 18 20 23 27 28
18 15 19 22 19
70 67 69 70 74 74
78 77 81 84 85 86 90
23 20 21 25 26 28 29 34
66 65 66 72 74
55 52 53 56 63 62
53 52 57 58 60 60
73 70 75 76 77 78 82
52 49 51 58 65
58 58 61 64 66 67 70
74 74 76 79 80 77
32 32 34 37 39 39
73 73 76 78 79 83
44 44 47 48 54
85 85 87 86 89 90
53 53 50 53 56 58 56
46 46 47 44 44
4 4 6 8 10 12 9 13
29 29 26 27 34
63 63 63 65 68
89 89 90 92 93 93 94 92
19 19 19 22 24 25 25
42 42 42 43 47
29 29 30 30 33 40
31 31 35 38 40
70 70 72 74 78 77
34 34 35 39 39
27 27 29 33 36 40
76 76 78 82 88
66 66 69 75 78
38 38 39 41 46 43
23 23 25 30 33 36 37 37
43 43 46 47 48 53 56 60
60 60 65 66 73
52 56 57 60 63 66 68
10 14 17 19 20 23 22
86 90 93 95 95
53 57 59 60 64
62 66 69 72 74 77 84
49 53 55 52 54
87 91 93 96 95 96 93
3 7 4 6 6
57 61 63 65 67 68 65 69
75 79 76 79 84
35 39 40 43 46 46 47
24 28 30 32 34 35 35 33
65 69 71 71 71
79 83 86 88 91 94 94 98
45 49 49 52 54 61
25 29 32 35 37 41 44 47
2 6 7 8 12 15 18 15
22 26 28 31 33 37 37
78 82 86 87 90 92 96
65 69 70 74 75 78 85
37 41 43 46 49 54 56 59
19 23 26 27 33 32
11 15 17 24 24
77 81 86 89 93
22 26 28 29 31 33 40 46
32 39 41 44 45 48 50 51
10 17 18 21 23 26 23
31 37 39 41 43 45 45
42 48 49 50 54
25 32 35 38 39 41 43 49
45 50 48 49 51 52 54
18 24 21 23 21
75 82 85 88 90 92 89 89
48 55 57 60 63 66 65 69
59 66 68 65 72
19 26 26 29 30 32 34 35
84 90 93 95 95 93
32 39 42 45 45 45
77 83 85 86 87 89 89 93
45 51 51 52 59
71 77 78 82 83 84 86 87
70 77 79 83 86 89 91 89
56 63 64 68 70 71 71
45 51 55 56 59 61 64 68
5 11 12 16 23
22 27 29 36 39
61 68 71 78 79 77
5 11 18 19 19
20 25 27 34 36 39 40 44
18 25 30 31 32 38
76 73 72 70 72
55 52 49 48 45 44 44
20 19 17 14 13 12 11 7
24 22 19 16 15 10
65 63 66 65 63 61 59 58
6 5 2 4 5
36 33 35 33 32 30 28 28
43 42 43 42 38
23 21 18 15 17 12
40 38 37 37 36 33
6 4 4 2 4
50 47 47 45 42 42
79 78 78 75 73 70 67 63
17 14 13 13 11 8 2
75 74 73 69 67 66 63
86 84 81 77 78
18 17 14 12 11 7 7
72 70 67 63 60 56
36 35 32 30 26 23 18
76 75 73 71 65 62
61 59 58 56 51 50 48 49
52 49 46 43 38 36 36
80 79 73 70 69 68 65 61
82 79 77 70 64
24 25 24 23 22
53 56 53 51 54
69 70 67 65 64 63 62 62
64 66 64 63 61 60 56
92 95 93 91 88 87 82
83 86 87 84 83 80
78 81 79 77 75 73 75 78
84 85 82 85 84 84
62 64 67 66 65 61
36 38 37 40 38 35 33 28
77 80 79 78 75 72 72 69
33 36 35 34 32 32 29 31
18 21 21 19 16 13 13
12 15 13 11 11 7
83 85 84 84 81 79 76 70
78 80 78 77 74 72 68 67
48 50 49 46 42 39 40
22 24 20 18 16 14 14
79 82 81 77 76 72
75 77 73 71 70 69 63
43 46 43 37 36
51 52 45 43 42 41 43
98 99 98 92 92
61 62 57 56 52
34 35 33 32 31 26 24 18
34 34 32 30 29 27 26 24
97 97 95 94 97
54 54 52 49 48 47 46 46
38 38 37 35 31
23 23 22 20 17 14 7
50 50 49 52 49 47
51 51 53 50 48 46 43 44
36 36 39 38 35 33 31 31
78 78 80 78 74
90 90 88 91 89 88 82
12 12 10 10 8 5
61 61 58 58 59
9 9 6 6 6
60 60 57 57 53
99 99 97 97 95 94 92 87
19 19 15 12 11 8 5 4
98 98 94 93 92 94
45 45 41 40 39 36 36
46 46 42 40 37 33
20 20 19 16 14 10 4
26 26 25 19 16 13 10 8
98 98 95 89 92
87 87 85 84 77 75 75
25 25 22 20 17 12 10 6
65 65 64 59 54
47 43 40 37 36 35 32 30
28 24 22 19 18 17 19
21 17 16 13 11 10 7 7
21 17 15 14 10
97 93 90 89 87 85 79
22 18 16 13 11 13 10 8
65 61 60 61 60 59 60
94 90 88 90 89 89
20 16 19 17 14 12 9 5
50 46 43 42 44 41 38 32
18 14 11 11 8 5
46 42 39 38 38 39
69 65 64 64 64
18 14 12 11 10 9 9 5
26 22 21 18 18 11
67 63 60 59 55 54 51 49
88 84 81 77 74 71 73
97 93 91 87 85 82 82
75 71 69 68 64 63 59
92 88 84 81 78 76 70
28 24 19 16 14 13 10 9
47 43 40 37 30 28 29
21 17 12 11 8 5 2 2
38 34 33 28 25 21
23 19 13 12 11 6
38 31 29 26 23 21 20 17
99 94 91 88 86 87
29 22 20 17 17
36 30 27 24 22 18
44 39 38 35 34 27
94 87 86 87 86
53 47 50 49 46 47
44 37 36 39 38 37 37
21 15 12 14 11 10 6
50 45 44 45 40
92 86 85 83 82 80 80 78
72 67 66 66 64 65
94 89 89 88 88
67 62 60 60 57 53
34 29 29 27 26 21
22 15 13 11 9 5 2 1
62 55 51 50 53
68 61 58 57 56 53 49 49
66 59 56 52 50 46
77 71 70 67 66 62 57
44 39 36 30 27
31 26 23 20 14 13 12 14
45 40 39 32 32
84 79 73 72 68
28 23 20 14 9
57 61 62 65 67 71 71
98 93 93 92 90 89 85
75 75 78 84 87 90 91 95
59 56 52 49 47
16 16 13 10 10
75 74 72 71 74
29 24 21 17 16 14 8
40 42 43 44 48
14 14 15 18 21 22 24 31
4 4 6 12 14 20
43 39 36 39 38
26 22 19 17 20
79 73 72 69 68 68 66
47 43 40 39 36 35 34
33 33 32 31 28 27 29
37 34 32 32 31 29
79 75 73 70 69 67 68 63
67 66 67 69 71 71 74
23 30 27 29 30 33 40
27 22 21 18 17 17 17
29 35 38 39 43 43
64 64 66 73 75 76
82 76 72 69 67 65
81 74 77 75 72 69 65
74 74 72 70 66 65
64 61 59 58 54 51 48 48
12 13 16 15 12 11 5
10 16 19 21 27 29 32 32
93 89 88 87 87 83
51 50 53 54 55 58 60 65
19 19 19 21 20
90 90 92 94 93 93
31 35 36 42 44 51
41 41 36 34 33 32 31
69 62 59 58 56 53 48
51 45 45 42 41 40 33
39 42 39 38 35 32 28 22
44 45 42 39 35 33 29
24 27 29 30 33 36 37
13 10 8 7 4 2
37 35 33 32 30 29 26 25
59 58 57 56 54 51 50
54 57 58 61 63 66 67 68
71 69 67 64 62 60 58
22 20 17 14 12 10
74 73 70 69 68
14 17 19 20 22
24 27 29 30 32 35 38 39
74 77 79 82 83 85 87
42 41 39 38 35 34 33
80 79 76 75 74
16 13 11 8 6 4 3 2
25 26 27 29 30 32
56 57 60 63 65 67
64 66 68 71 74 75
68 69 72 75 77 79
37 40 42 43 44 45 46 47
54 55 58 61 63 64 67 70
81 82 84 86 89 90 92
76 77 78 81 82 85 87 89
9 12 15 16 17 18
21 24 25 26 27 29 30 33
34 35 37 40 43 45 48 50
53 52 51 49 48 47 44
82 85 87 89 91
83 84 85 88 91 94 95
57 58 61 64 66 68 71 73
9 11 14 16 18
77 76 73 72 71 70
17 16 13 12 11 10 8 5
58 60 62 65 68 71 74 76
65 62 59 58 57 56 55
97 95 92 91 90 88 87 84
88 86 85 82 81
23 26 28 30 32 33 36 37
1 2 3 4 5 6 8
51 53 56 59 61
4 6 9 11 12
90 87 86 83 80 77 74
50 52 54 57 60
12 13 14 17 19
89 88 87 85 84 81 80 79
13 10 8 6 4 2
20 21 22 24 26 27
71 70 69 67 65 64
14 13 12 10 8 6
24 25 27 30 32 34 37 38
84 85 87 88 90 91 93 96
55 54 51 49 48 45 44
96 94 91 89 86 85 82 80
57 54 53 52 49 47 44 43
44 47 50 51 53 55 57
24 25 28 30 33 35 37 40
65 67 69 71 73 75 77 79
30 33 36 37 38 41 43 46
27 29 31 33 35 38 40
51 53 55 56 59 60
18 21 24 26 27 29 31 33
40 43 46 49 50 51
69 68 65 63 60 59
16 15 13 10 9 6 4 3
76 79 82 83 84 85
29 27 26 25 22
63 60 57 54 53 50 47
16 17 19 20 21 22 24
79 76 75 73 72 71 70 67
30 31 32 35 38 40 41 44
49 51 52 53 56
66 68 69 70 71
73 75 76 79 80 83 85 88
47 46 44 41 39 37 34 31
80 77 75 74 73 71 69
47 46 43 40 37 35
39 42 45 48 49 52 55 58
20 22 24 26 28 30 31 34
88 89 92 94 96 98
64 61 60 59 56 55
39 42 44 46 47
77 79 80 83 86 88 89
80 77 74 73 71 68
26 25 22 19 16 15 14 11
36 33 30 29 28 27 24 22
54 57 58 60 63 65 68
49 47 44 43 41 38
23 26 28 30 32
72 74 76 77 80 82
41 38 36 35 34 33
56 54 51 48 46
66 67 69 71 73 75 76 77
19 18 15 13 12
21 18 16 15 12 10 9 8
87 85 83 81 79 78
47 48 51 52 53 54 55
57 58 59 61 63 64 66 68
46 49 52 54 57
5 7 8 9 11 12 13 16
33 35 36 37 39 42 45 46
74 71 70 67 65
23 26 29 30 32 33 34
60 63 65 66 67 68 70
75 73 72 71 68 67
46 45 43 42 41 38 37 36
13 12 9 8 5
36 38 41 43 45 47 49
8 9 11 12 13 15
26 29 30 32 34 35 37 40
80 77 75 74 72
13 15 18 21 24 26
17 16 15 12 11 9 6 5
58 55 54 52 51 49 47 45
79 76 75 74 73 71
69 72 73 75 78 81 82 84
65 67 68 71 73 75 77
20 18 16 15 14
61 63 65 68 71
81 80 79 78 75 72
62 64 67 70 73 74 75
54 57 60 63 66 69
35 32 30 27 25
33 35 36 39 40
28 25 22 20 19 17 14 13
96 93 92 89 88 85 83
68 69 72 75 78 80 82
43 44 46 49 51 54
99 98 95 93 91
59 62 63 65 68 69
35 34 32 31 30 27
97 94 92 91 89 88
19 20 22 25 28
38 36 34 32 31 29 27
47 49 50 53 56 59 62 63
86 89 92 94 95
18 20 21 22 25 26
72 74 76 78 79 81
85 86 87 89 90 93 95
80 83 84 86 87 90 91
8 9 11 13 15
24 26 29 32 35 38
58 55 53 52 51 49
92 89 87 86 84 83
97 95 93 92 90 89 86 85
74 73 70 69 66 63 61 58
74 71 68 67 65 62 61
50 53 56 57 58
55 56 58 60 63 64
87 85 83 82 79 78 75 73
59 62 64 66 69
19 20 23 25 26 28 30 32
93 91 90 88 86
93 92 90 88 86 84 81 78
39 38 36 33 30 29 27 26
81 84 86 87 90 92 93 95
45 42 39 38 35 34
22 19 18 15 13 11 8 5
41 40 39 36 35 33
56 54 53 51 48 45 42 41
74 72 69 68 66 63
66 69 72 75 77 79
74 72 69 68 67 64 62
2 5 8 11 12 13 16
87 84 83 82 80 77 75
10 9 8 6 5 3 2
24 25 27 30 31 32
74 73 71 68 67 64
9 11 12 14 16 17 20 21
75 72 71 68 65 63 62 60
43 41 39 38 35 33 32
85 82 80 77 74 73 71
81 80 77 75 73 70 67
62 63 64 65 68
46 44 42 39 37 36 33
77 76 73 71 68 67 64
68 70 73 76 78 79 82
76 73 70 68 67 64 62 60
47 49 52 53 54 55 58
32 29 27 25 22 21 19
76 73 72 70 69 66 65
33 32 31 29 27 25 23 21
4 7 9 12 13 15 16 17
77 76 75 73 70 69
14 11 8 7 4 2
26 24 21 19 16 15
40 39 38 36 34
58 61 64 66 67 70
69 72 73 74 75
89 88 86 85 82 80 77
87 88 90 91 92
52 54 55 56 59
4 6 9 11 14 15 18 21
22 25 28 30 31
17 14 12 11 9 6
73 72 71 70 67 64 61 60
63 64 66 69 70 72 74 75
64 67 68 70 73 76 78
96 95 92 90 89 86
39 38 36 35 34 32 31
74 73 71 68 67 64 61 60
88 87 84 81 80 79 78
51 52 53 56 59 62
26 25 24 23 20 18 17
45 47 48 51 54 55 58 60
46 45 42 41 40 38
85 83 81 78 77 76 75
57 56 54 52 50 48 45 44
15 16 19 20 21
78 80 82 85 87 89 91
23 26 28 29 31 32 35
87 86 84 83 81 80
52 51 48 46 44 41
97 94 93 92 91 88
46 47 49 52 54 56 58 59
60 58 56 54 52 50
11 10 7 6 4 3 2 1
57 59 61 63 64
38 40 43 45 46
61 63 66 68 71"
        .to_string();
    let binding: Vec<_> = input.split('\n').collect();

    let mut res: Vec<Vec<&str>> = vec![];

    for i in binding {
        res.push(i.split(' ').collect());
    }

    let mut numres: Vec<Vec<i32>> = vec![];

    for i in res {
        let mut temp: Vec<i32> = vec![];
        for j in i {
            temp.push(FromStr::from_str(j).unwrap());
        }
        numres.push(temp);
    }

    let mut counter: i32 = 0;

    for i in numres {
        if res_func(&i) == true {
            counter += 1;
        }
    }

    println!("Number of valid reports: {}", counter);
}
