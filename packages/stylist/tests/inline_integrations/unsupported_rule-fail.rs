fn main() {
    let is_an_expression = "black";
    let _ = stylist::css! {
        @page {
            margin: 1cm;
        }
        @property property-name {
            syntax: "<color>";
            inherits: false;
            initial-value: #c0ffee;
        }
        @completely-unknown {
            some-attribute: foo-value;
        }
    };
}
