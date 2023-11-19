package school

default is_student = false

is_student {
    startswith(input.email, "s")
}
