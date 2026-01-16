Our area function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior
more closely to our Rectangle struct because it won’t work with any other type. Let’s look at how we can continue to refactor this code
by turning the area function into an area method defined on our Rectangle type

Unlike functions, methods are defined within
the context of a struct (or an enum or a trait object) and their first parameter
is always self, which represents the instance of the struct the
method is being called on.
