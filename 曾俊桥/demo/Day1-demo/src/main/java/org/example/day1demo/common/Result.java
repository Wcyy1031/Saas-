package org.example.day1demo.common;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
public class Result<T> {
    private Integer code;
    private String msg;
    private T data;

    public static <T> Result<T> success(T data){
        return Result.<T>builder().code(200).msg("success").data(data).build();
    }

    public static <T> Result<T> error(String msg){
        return Result.<T>builder().code(500).msg(msg).data(null).build();
    }
}
