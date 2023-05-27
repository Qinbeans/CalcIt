export interface Constant {
    Integer?: string
    Float?: string
    Binary?: string
    Hexadecimal?: string
    Octal?: string
}

export enum Target {
    Integer = "Integer",
    Float = "Float",
    Binary = "Binary",
    Hexadecimal = "Hexadecimal",
    Octal = "Octal",
}

export const evaluateConstant = (constant: Constant): string => {
    if (constant.Integer) return constant.Integer
    if (constant.Float) return constant.Float
    if (constant.Binary) return constant.Binary
    if (constant.Hexadecimal) return constant.Hexadecimal
    if (constant.Octal) return constant.Octal
    return ''
}