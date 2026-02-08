package com.mycompany.app;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelHandler;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.util.CharsetUtil;

@ChannelHandler.Sharable
public class EchoClientHandler extends SimpleChannelInboundHandler<ByteBuf> {

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        // ctx.writeAndFlush(Unpooled.copiedBuffer("Netty rocks!", CharsetUtil.UTF_8));

        String str = """
AWS hasn’t publicly documented exact pieces of disk hardware (model numbers, vendors, etc.) that back S3 Express One Zone specifically, but here’s what is known/strongly implied about the storage media Amazon uses for this storage class:

🧱 1. It’s S3 Object Storage with Purpose-Built Hardware

S3 Express One Zone is a high-performance S3 storage class where data resides in a single AWS Availability Zone and is stored redundantly across multiple devices within that AZ for durability and availability.

AWS describes the class as using “purpose-built hardware” optimized for consistent single-digit millisecond latency and high throughput — meaning the infrastructure is architected for performance, not just cheap bulk capacity.

🔍 2. Likely Backed by SSDs (Solid-State Drives)

While AWS does not explicitly state the hardware type, multiple independent analysts and discussing community consensus strongly suggest high-performance S3 storage classes—including Express One Zone—are backed by SSDs or similar flash-based media:

Commenters note that to achieve single-digit millisecond access at scale, AWS would almost certainly use SSD-class storage versus spinning HDDs (which have much higher latency).

Other deep dives on S3 Express indicate “performance … backed by new storage media,” with SSDs being the most plausible technology.

SSDs make sense for this class because SSD performance characteristics (low latency, high IOPS) align well with the goals of Express One Zone.

🧠 3. Not In-Memory or RAM-Only

Express One Zone is not an in-memory cache like ElastiCache or other RAM-based stores. Its pricing per GB and usage patterns imply persistent SSD or equivalent storage rather than volatile RAM.

📦 4. How It Compares to Other S3 Storage Foundations

For context, the broader S3 service historically has been built on scalable, distributed object storage infrastructure with a mix of highly optimized servers and storage arrays. AWS does not publish the exact hardware details publicly, but the overall service operates across many data centers and uses massive distributed systems to manage data durability and scalability.

Express One Zone is a variant of that infrastructure, tuned for latency and co-location with compute, rather than geographical redundancy or maximum long-term durability.

💡 Summary (What S3 Express One Zone Uses)

✔️ Stores objects in single AZ with internal replication across multiple devices within that AZ (for durability/availability).
✔️ Uses optimized, purpose-built storage hardware rather than generic HDD arrays.
✔️ Most likely backed by SSDs or flash-based media to deliver consistent single-digit millisecond latency.
✔️ Not an in-memory store — persistent storage.

If you need details on latency performance patterns or how this compares with other S3 classes (Standard, Intelligent-Tiering, etc.), I can dive into that too!
                """;
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
