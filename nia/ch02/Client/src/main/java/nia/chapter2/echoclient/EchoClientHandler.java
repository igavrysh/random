package nia.chapter2.echoclient;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelHandler;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.util.CharsetUtil;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.Scanner;

@ChannelHandler.Sharable
public class EchoClientHandler extends SimpleChannelInboundHandler<ByteBuf> {

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        // ctx.writeAndFlush(Unpooled.copiedBuffer("Netty rocks!", CharsetUtil.UTF_8));
        String fileName = "adventures-of-huckleberry-finn-by-mark-twain.txt";
        InputStream is = getClass().getClassLoader().getResourceAsStream(fileName);
        if (is == null) {
            throw new IllegalArgumentException(("File not found " + fileName));
        }
        String str = null;
        try (Scanner scanner = new Scanner(is, StandardCharsets.UTF_8)) {
            str = scanner.useDelimiter("\\A").next();
        }
        ctx.writeAndFlush(Unpooled.copiedBuffer(str, CharsetUtil.UTF_8));
    }

    @Override
    protected void channelRead0(ChannelHandlerContext ctx, ByteBuf in) throws Exception {
        System.out.println(" === Client received: ==================================\n" + in.toString(CharsetUtil.UTF_8));
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        System.out.println(" === !!! Exception caught: ==================================\n");

        cause.printStackTrace();
        ctx.close();
    }
}
