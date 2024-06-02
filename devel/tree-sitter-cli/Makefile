PORTNAME=	tree-sitter-cli
DISTVERSIONPREFIX=	v
DISTVERSION=	0.22.6
CATEGORIES=	devel

MAINTAINER=	mikael@FreeBSD.org
COMMENT=	Incremental parser generator
WWW=		https://tree-sitter.github.io/

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/../LICENSE

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	tree-sitter
GH_PROJECT=	tree-sitter

CARGO_CARGOLOCK=	${WRKSRC}/../Cargo.lock

WRKSRC=	${WRKDIR}/${GH_PROJECT}-${DISTVERSION}/cli

PLIST_FILES=	bin/tree-sitter

.include <bsd.port.mk>
